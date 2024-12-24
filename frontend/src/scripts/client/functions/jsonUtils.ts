const TYPE_IDENTIFIER = "__type";
const TYPES = {
  UINT8ARRAY: "Uint8Array",
  // Add more special types here as needed
};

function customReplacer(_key: string, value: unknown): unknown {
  if (value instanceof Uint8Array) {
    return {
      [TYPE_IDENTIFIER]: TYPES.UINT8ARRAY,
      data: Array.from(value),
    };
  }
  return value;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function customReviver(_key: string, value: any): any {
  if (value && typeof value === "object" && value[TYPE_IDENTIFIER]) {
    switch (value[TYPE_IDENTIFIER]) {
      case TYPES.UINT8ARRAY:
        return new Uint8Array(value.data);
      default:
        return value;
    }
  }
  return value;
}

export function jsonStringify(obj: unknown): string {
  return JSON.stringify(obj, customReplacer);
}

export function jsonParse<T = unknown>(text: string): T {
  return JSON.parse(text, customReviver);
}
