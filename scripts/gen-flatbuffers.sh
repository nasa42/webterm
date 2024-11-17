set -e

SCHEMA_PATH="./flatbuffers/*.fbs"
SHARED_PATH="./shared/src/generated/flatbuffers_schema/"
FRONTEND_PATH="./frontend/src/generated/flatbuffers_schema/"

rm -rf $SHARED_PATH
flatc --rust --rust-module-root-file -o $SHARED_PATH $SCHEMA_PATH

rm -rf $FRONTEND_PATH
flatc --ts -o $FRONTEND_PATH $SCHEMA_PATH
