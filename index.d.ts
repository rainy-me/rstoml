// This is written by hand. So sad I don't know why the type gen doesn't for me.

/**
 * Parse input string as TOML
 */
export function parse(text: string): unknown

/**
 * Parse input buffer as TOML
 */
export function parseBuffer(buffer: Buffer): unknown

/**
 * Stringify option
 */
export interface StringifyOption {
  /**
   * Serialize the given data structure as a "pretty" String of TOML.
   */
  pretty: boolean
}
/**
 * Converts a JavaScript Object to TOML string.
 */
export function stringify(obj: object, options?: StringifyOption): string
