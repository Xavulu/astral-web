/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} in_put
* @param {Uint8Array} out_put
*/
export function b3_checksum(in_put: Uint8Array, out_put: Uint8Array): void;
/**
* @param {Uint8Array} in_put
* @param {Uint8Array} out_put
* @param {Uint8Array} hash
* @returns {boolean}
*/
export function b3_checksum_verify(in_put: Uint8Array, out_put: Uint8Array, hash: Uint8Array): boolean;
/**
* @param {Uint8Array} in_put
* @param {string} pass
* @param {boolean} armor
* @returns {Uint8Array}
*/
export function encrypt_data(in_put: Uint8Array, pass: string, armor: boolean): Uint8Array;
/**
* @param {Uint8Array} in_put
* @param {string} pass
* @returns {Uint8Array}
*/
export function decrypt_data(in_put: Uint8Array, pass: string): Uint8Array;
/**
* @param {Uint8Array} in_put
* @returns {Uint8Array}
*/
export function compress_data(in_put: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} in_put
* @returns {Uint8Array}
*/
export function decompress_data(in_put: Uint8Array): Uint8Array;
