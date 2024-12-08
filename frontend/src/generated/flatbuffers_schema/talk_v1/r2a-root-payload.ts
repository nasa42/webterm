// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import { EmptyTable } from './empty-table.js';
import { R2aError } from './r2a-error.js';
import { R2aFromFrontend } from './r2a-from-frontend.js';


export enum R2aRootPayload {
  NONE = 0,
  Error = 1,
  FromFrontend = 2,
  RelayShuttingDown = 4
}

export function unionToR2aRootPayload(
  type: R2aRootPayload,
  accessor: (obj:EmptyTable|R2aError|R2aFromFrontend) => EmptyTable|R2aError|R2aFromFrontend|null
): EmptyTable|R2aError|R2aFromFrontend|null {
  switch(R2aRootPayload[type]) {
    case 'NONE': return null; 
    case 'Error': return accessor(new R2aError())! as R2aError;
    case 'FromFrontend': return accessor(new R2aFromFrontend())! as R2aFromFrontend;
    case 'RelayShuttingDown': return accessor(new EmptyTable())! as EmptyTable;
    default: return null;
  }
}

export function unionListToR2aRootPayload(
  type: R2aRootPayload, 
  accessor: (index: number, obj:EmptyTable|R2aError|R2aFromFrontend) => EmptyTable|R2aError|R2aFromFrontend|null, 
  index: number
): EmptyTable|R2aError|R2aFromFrontend|null {
  switch(R2aRootPayload[type]) {
    case 'NONE': return null; 
    case 'Error': return accessor(index, new R2aError())! as R2aError;
    case 'FromFrontend': return accessor(index, new R2aFromFrontend())! as R2aFromFrontend;
    case 'RelayShuttingDown': return accessor(index, new EmptyTable())! as EmptyTable;
    default: return null;
  }
}
