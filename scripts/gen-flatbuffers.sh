set -e

SCHEMAS=("handshake_v1" "talk_v1")
SHARED_BASE_PATH="./shared/src/generated/flatbuffers_schema/"
FRONTEND_BASE_PATH="./frontend/src/generated/flatbuffers_schema/"

for SCHEMA in "${SCHEMAS[@]}"; do
  SCHEMA_PATH="./schema/$SCHEMA.fbs"
  SHARED_PATH="${SHARED_BASE_PATH}${SCHEMA}/"
  FRONTEND_PATH="${FRONTEND_BASE_PATH}${SCHEMA}/"

  rm -rf $SHARED_PATH
  flatc --rust --rust-module-root-file --gen-all -o $SHARED_PATH $SCHEMA_PATH

  rm -rf $FRONTEND_PATH
  flatc --ts --gen-all -o $FRONTEND_PATH $SCHEMA_PATH
done
