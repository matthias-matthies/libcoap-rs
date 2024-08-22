// SPDX-License-Identifier: BSD-2-CLAUSE

use version_compare::{Cmp, Version};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(dtls_ec_jpake_support)");
    println!("cargo::rustc-check-cfg=cfg(dtls_cid_support)");
    if let Ok(libcoap_version) = std::env::var("DEP_COAP_3_LIBCOAP_VERSION") {
        let version = Version::from(libcoap_version.as_ref()).expect("invalid libcoap version");
        match version.compare(Version::from("4.3.5rc3").unwrap()) {
            Cmp::Gt | Cmp::Eq => {
                println!("cargo:rustc-cfg=dtls_ec_jpake_support");
                println!("cargo:rustc-cfg=dtls_cid_support");
            },
            _ => {},
        }
    }
}
