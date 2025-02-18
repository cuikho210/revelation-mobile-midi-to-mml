#!/usr/bin/bash

provider="huggingface"

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -p|--provider) provider="$2"; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

# Provider-specific configs
if [ "$provider" = "huggingface" ]; then
    MODEL="Qwen/Qwen2.5-Coder-32B-Instruct"
    API_URL="https://api-inference.huggingface.co/models/$MODEL/v1/chat/completions"
    AUTH_HEADER="Authorization: Bearer $HF_TOKEN"
elif [ "$provider" = "openai" ]; then
    MODEL="o3-mini"
    API_URL="https://api.openai.com/v1/chat/completions"
    AUTH_HEADER="Authorization: Bearer $OPENAI_API_KEY"
else
    echo "Invalid provider. Use 'huggingface' or 'openai'"
    exit 1
fi

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
        -H "$AUTH_HEADER" \
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
            ]
        }"
    )

    commit_message=$(echo "$response" | jq -r '.choices[0].message.content' 2>/dev/null)

    if [ -z "$commit_message" ] || [ "$commit_message" = "null" ]; then
        echo "Error: Empty or null commit message. Full response:" >&2
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
