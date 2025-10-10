LAST_TAG=$(curl -sSL https://api.github.com/repos/alshdavid/procmon/releases/latest | jq ".tag_name" | cut -d "." -f 3)
if [ "$LAST_TAG" = "" ]; then
  LAST_TAG="0"
fi
declare -i var="$LAST_TAG"
var=$var+1
TAG="0.0.$var"
echo $TAG