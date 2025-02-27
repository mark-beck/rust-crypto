use robusta_jni::bridge;

#[bridge]
/// This module contains the JNI bindings for key generation in Android.
pub(crate) mod jni {
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::sys::jbyteArray,
        jni::{errors::Result as JniResult, objects::AutoLocal, JNIEnv},
    };

    /// Represents a key in Java's `java.security` package.
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub(crate) struct Key<'env: 'borrow, 'borrow> {
        #[instance]
        pub(crate) raw: AutoLocal<'env, 'borrow>,
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(javax.crypto)]
    pub(crate) struct SecretKey<'env: 'borrow, 'borrow> {
        #[instance]
        pub(crate) raw: AutoLocal<'env, 'borrow>,
    }

    /// Represents a public key in Java's `java.security` package.
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub(crate) struct PublicKey<'env: 'borrow, 'borrow> {
        #[instance]
        pub(crate) raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> PublicKey<'env, 'borrow> {
        /// Converts the public key to its string representation.
        pub(crate) extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}

        /// Retrieves the algorithm used by the public key.
        pub(crate) extern "java" fn getAlgorithm(&self, _env: &JNIEnv) -> JniResult<String> {}

        /// Retrieves the encoded public key in unknown format.
        pub(crate) fn getEncoded(&self, env: &JNIEnv) -> JniResult<Vec<u8>> {
            let output = env.call_method(self.raw.as_obj(), "getEncoded", "()[B", &[])?;

            let output_array = output.l()?.into_inner() as jbyteArray;
            let output_vec = env.convert_byte_array(output_array)?;

            Ok(output_vec)
        }
    }

    /// Represents a private key in Java's `java.security` package.
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub(crate) struct PrivateKey<'env: 'borrow, 'borrow> {
        #[instance]
        pub(crate) raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> PrivateKey<'env, 'borrow> {
        /// Converts the private key to its string representation.
        pub(crate) extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}

        /// Retrieves the algorithm used by the private key.
        pub(crate) extern "java" fn getAlgorithm(&self, _env: &JNIEnv) -> JniResult<String> {}
    }
}
