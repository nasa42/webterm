class BigIntLike {
  protected _bigint: bigint;

  constructor(bigint: bigint) {
    this._bigint = bigint;
  }

  int(): bigint {
    return this._bigint;
  }
}

export class ActivityId extends BigIntLike {}

export class FrontendId extends BigIntLike {}
