// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import { A2rHandshakeError } from './a2r-handshake-error.js';
import { A2rHandshakeRequestConnection } from './a2r-handshake-request-connection.js';


export enum A2rHandshakeRootPayload {
  NONE = 0,
  Error = 1,
  RequestConnection = 2
}

export function unionToA2rHandshakeRootPayload(
  type: A2rHandshakeRootPayload,
  accessor: (obj:A2rHandshakeError|A2rHandshakeRequestConnection) => A2rHandshakeError|A2rHandshakeRequestConnection|null
): A2rHandshakeError|A2rHandshakeRequestConnection|null {
  switch(A2rHandshakeRootPayload[type]) {
    case 'NONE': return null; 
    case 'Error': return accessor(new A2rHandshakeError())! as A2rHandshakeError;
    case 'RequestConnection': return accessor(new A2rHandshakeRequestConnection())! as A2rHandshakeRequestConnection;
    default: return null;
  }
}

export function unionListToA2rHandshakeRootPayload(
  type: A2rHandshakeRootPayload, 
  accessor: (index: number, obj:A2rHandshakeError|A2rHandshakeRequestConnection) => A2rHandshakeError|A2rHandshakeRequestConnection|null, 
  index: number
): A2rHandshakeError|A2rHandshakeRequestConnection|null {
  switch(A2rHandshakeRootPayload[type]) {
    case 'NONE': return null; 
    case 'Error': return accessor(index, new A2rHandshakeError())! as A2rHandshakeError;
    case 'RequestConnection': return accessor(index, new A2rHandshakeRequestConnection())! as A2rHandshakeRequestConnection;
    default: return null;
  }
}
