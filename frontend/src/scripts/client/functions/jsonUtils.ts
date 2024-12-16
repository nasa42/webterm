const TYPE_IDENTIFIER = "__type";
const TYPES = {
  UINT8ARRAY: "Uint8Array",
  // Add more special types here as needed
};

function customReplacer(_key: string, value: any): any {
  if (value instanceof Uint8Array) {
    return {
      [TYPE_IDENTIFIER]: TYPES.UINT8ARRAY,
      data: Array.from(value),
    };
  }
  return value;
}

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

export function jsonStringify(obj: any): string {
  return JSON.stringify(obj, customReplacer);
}

export function jsonParse<T = any>(text: string): T {
  return JSON.parse(text, customReviver);
}
