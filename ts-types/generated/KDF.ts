// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Argon2Options } from "./Argon2Options";

/**
 * Enum representing different secure key derivation functions
 *
 * For clients Argon2d is recommended as it offers excellent brute force resistance.
 * ```
 * # use crypto_layer::prelude::*;
 * /// Taken from KeePass (12.03.2025)
 * /// https://keepass.info/help/base/security.html
 * let client_kdf = KDF::Argon2id(Argon2Options {
 *            memory: 1048576,
 *            iterations: 2,
 *            parallelism: 4,
 *        });
 * ```
 *
 * For servers Argon2id is recommended as it also offers some side channel attack resistance.
 * ```
 * # use crypto_layer::prelude::*;
 * /// Taken from Password Storage Cheat Sheet (12.03.2025)
 * /// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
 * let server_kdf = KDF::Argon2id(Argon2Options {
 *            memory: 19456,
 *            iterations: 2,
 *            parallelism: 1,
 *        });
 * ```
 */
export type KDF = { "Argon2d": Argon2Options } | { "Argon2id": Argon2Options };
