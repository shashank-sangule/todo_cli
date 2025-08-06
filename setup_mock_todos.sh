#!/bin/bash

echo "🚀 Setting up comprehensive mock todos..."

CLI="./target/release/todo"
TEST_FILE="test_todos.json"

echo "🗑️ Clearing existing todos..."
$CLI --file $TEST_FILE clear-list

echo "📝 Adding diverse mock todos..."

$CLI --file $TEST_FILE add "Fix authentication security vulnerability" \
    --due-date "yesterday" \
    --priority "high" \
    --description "Critical security patch needed for user login system" \
    --tags "security","critical","backend"

$CLI --file $TEST_FILE add "Submit annual compliance report" \
    --due-date "today" \
    --priority "high" \
    --description "SEC filing deadline - cannot be delayed" \
    --tags "legal","compliance","deadline"

$CLI --file $TEST_FILE add "Present quarterly results to board" \
    --due-date "tomorrow" \
    --priority "high" \
    --description "Q4 performance summary with growth projections" \
    --tags "presentation","executive","quarterly"

$CLI --file $TEST_FILE add "Deploy hotfix to production" \
    --due-date "2d" \
    --priority "high" \
    --description "Fix payment processing bug affecting 15% of users" \
    --tags "deployment","bugfix","production"

$CLI --file $TEST_FILE add "Code review for authentication module" \
    --due-date "3d" \
    --priority "medium" \
    --description "Review PR #156 - OAuth 2.0 integration" \
    --tags "code-review","authentication","security"

$CLI --file $TEST_FILE add "Update API documentation" \
    --due-date "1w" \
    --priority "medium" \
    --description "Add examples for v2.1 endpoints and deprecation notices" \
    --tags "documentation","api","maintenance"

$CLI --file $TEST_FILE add "Conduct team retrospective meeting" \
    --due-date "next friday" \
    --priority "medium" \
    --description "Sprint 23 retrospective - discuss blockers and improvements" \
    --tags "meeting","agile","team"

$CLI --file $TEST_FILE add "Schedule annual health checkup" \
    --due-date "2w" \
    --priority "medium" \
    --description "Book appointment with Dr. Smith for physical exam" \
    --tags "health","personal","appointment"

$CLI --file $TEST_FILE add "Plan mom's 60th birthday party" \
    --due-date "3w" \
    --priority "medium" \
    --description "Venue, catering, guest list, decorations" \
    --tags "family","celebration","planning"

$CLI --file $TEST_FILE add "Complete Rust advanced patterns course" \
    --due-date "1m" \
    --priority "low" \
    --description "Finish modules 8-12 on lifetimes and async programming" \
    --tags "learning","rust","professional-development"

$CLI --file $TEST_FILE add "Write blog post about CLI development" \
    --due-date "2w" \
    --priority "low" \
    --description "Share experience building todo app with Rust and clap" \
    --tags "writing","blog","rust","sharing"

$CLI --file $TEST_FILE add "Organize digital photo collection" \
    --priority "low" \
    --description "Sort and backup photos from 2023 vacation trips" \
    --tags "personal","organization","digital-cleanup"

$CLI --file $TEST_FILE add "Learn to play piano" \
    --priority "low" \
    --description "Start with basic scales and simple songs" \
    --tags "hobby","music","personal-growth"

$CLI --file $TEST_FILE add "Read 'The Rust Programming Language' book" \
    --description "Deep dive into ownership, borrowing, and lifetimes" \
    --tags "learning","rust","book"

$CLI --file $TEST_FILE add "Research weekend hiking trails" \
    --description "Find dog-friendly trails within 2 hours of city" \
    --tags "outdoor","hiking","weekend","dogs"

echo "✅ Mock todos created successfully!"
echo "📊 Use these commands to test your CLI:"
echo ""
echo "List all todos:"
echo "  $CLI --file $TEST_FILE list"
echo ""
echo "Filter by priority:"
echo "  $CLI --file $TEST_FILE list --priority high"
echo ""
echo "Show overdue items:"
echo "  $CLI --file $TEST_FILE list --overdue"
echo ""
echo "Show today's tasks:"
echo "  $CLI --file $TEST_FILE list --due-today"
