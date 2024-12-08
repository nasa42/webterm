set -e

echo "Generating with $(flatc --version)"

SCHEMAS=("handshake_v1" "talk_v1")
SHARED_BASE_PATH="./core/src/generated/flatbuffers_schema/"
FRONTEND_BASE_PATH="./frontend/src/generated/flatbuffers_schema/"

for SCHEMA in "${SCHEMAS[@]}"; do
  SCHEMA_PATH="./schema/$SCHEMA.fbs"
  SHARED_PATH="${SHARED_BASE_PATH}${SCHEMA}/"
  FRONTEND_PATH="${FRONTEND_BASE_PATH}${SCHEMA}/"

  rm -rf $SHARED_PATH
  echo "Generating FlatBuffers for $SCHEMA in Rust"
  flatc --rust --rust-module-root-file --gen-all -o $SHARED_PATH $SCHEMA_PATH

  rm -rf $FRONTEND_PATH
  echo "Generating FlatBuffers for $SCHEMA in TypeScript"
  flatc --ts --gen-all -o $FRONTEND_PATH $SCHEMA_PATH
done

echo "Done"
