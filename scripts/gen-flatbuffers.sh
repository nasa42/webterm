set -e

flatc --rust -o ./shared/src/generated/ ./flatbuffers/schema.fbs
mv ./shared/src/generated/schema_generated.rs ./shared/src/generated/flatbuffers_schema.rs

rm -rf ./frontend/src/generated/flatbuffers_schema
flatc --ts -o ./frontend/src/generated/flatbuffers_schema ./flatbuffers/schema.fbs
