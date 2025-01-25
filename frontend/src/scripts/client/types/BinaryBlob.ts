export class BinaryBlob {
  protected _data: Uint8Array;

  constructor(data: Uint8Array) {
    this._data = data;
  }

  data(): Uint8Array {
    return this._data;
  }
}
