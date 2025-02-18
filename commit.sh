#!/usr/bin/bash

MODEL="Qwen/Qwen2.5-Coder-32B-Instruct"
API_URL="https://api-inference.huggingface.co/models/$MODEL/v1/chat/completions"

git_diff=$(git diff --cached | tr -d '\000-\037' | jq -Rs .)

if [ -z "$git_diff" ]; then
    echo "No staged changes to commit."
    exit 1
fi

system_message=$(cat <<'EOF' | jq -Rs .
Analyze the git diff and generate a descriptive git commit message following the Conventional Commits format:

<type>(<scope>): <description>

Types:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- refactor: A code change that neither fixes a bug nor adds a feature
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools/libraries
- perf: A code change that improves performance
- ci: Changes to CI configuration files and scripts
- build: Changes that affect the build system or external dependencies

Guidelines:
1. The description should be clear and meaningful
2. Use imperative, present tense ("add" not "added")
3. Don't capitalize first letter
4. No period at the end
5. Include the scope/module name in parentheses when applicable
6. Add breaking change warning with BREAKING CHANGE: prefix if needed
7. Can include a longer description body after the first line if necessary

Examples:

```
feat(auth): add OAuth2 authentication support

BREAKING CHANGE: new authentication flow requires client secret
Additional implementation details and migration guide here
```

```
fix(api): resolve null pointer in user lookup
```

```
docs(readme): update installation instructions
```

```
refactor(core): restructure database connection logic

- Separate connection pool management
- Add retry mechanism
- Improve error handling
```
EOF
)

function get_commit_message() {
    response=$(
        curl -s "$API_URL" \
        -X "POST" \
        -H "Authorization: Bearer $HF_TOKEN" \
        -H "Content-Type: application/json" \
        -H "x-use-cache: false" \
        -d "{
            \"model\": \"$MODEL\",
            \"messages\": [
                {
                    \"role\": \"system\",
                    \"content\": ${system_message}
                },
                {
                    \"role\": \"user\",
                    \"content\": ${git_diff}
                }
            ],
            \"temperature\": 0.5,
            \"max_tokens\": 2048,
            \"top_p\": 0.7,
            \"stream\": false
        }"
    )

    commit_message=$(echo "$response" | jq -r '.choices[0].message.content' 2>/dev/null)

    if [ -z "$commit_message" ] || [ "$commit_message" = "null" ]; then
        echo "Error: Empty or null commit message. Full response:" >&2
        echo "$response" >&2
        exit 1
    fi

    if [ $? -ne 0 ]; then
        echo "Error parsing response with jq. Full response:" >&2
        echo "$response" >&2
        exit 1
    fi

    echo "$commit_message" | sed 's/^"//;s/"$//'
}

while true; do
    commit_message=$(get_commit_message)
    echo "Suggested commit message:"
    echo "$commit_message"

    read -p "Do you accept this commit message? (y/n): " confirm
    case $confirm in
        [Yy]*)
            git commit -S -m "$commit_message"
            exit 0
            ;;
        [Nn]*)
            echo "Retrying..."
            ;;
        *)
            echo "Please answer y or n."
            ;;
    esac
done
