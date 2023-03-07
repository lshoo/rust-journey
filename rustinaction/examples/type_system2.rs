//! https://rust-unofficial.github.io/patterns/functional/generics-type-classes.html
//!

mod nfs {
    #[derive(Clone)]
    pub(crate) struct AuthInfo(pub String);
}

mod bootp {
    pub(crate) struct AuthInfo();
}

mod proto_trait {
    use crate::{bootp, nfs};
    use std::path::{Path, PathBuf};

    pub(crate) trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub(crate) fn new(auth: nfs::AuthInfo, mount_point: PathBuf) -> Self {
            Self { auth, mount_point }
        }

        pub(crate) fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp();

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}

use std::path::{Path, PathBuf};

use proto_trait::ProtoKind; // keep internal to prevent impls
pub use proto_trait::{Bootp, Nfs}; // re-export so callers can see them

struct FileDownloadRequest<P: ProtoKind> {
    file_name: PathBuf,
    protocol: P,
}

impl<P: ProtoKind> FileDownloadRequest<P> {
    pub fn new(file_name: PathBuf, protocol: P) -> Self {
        Self {
            file_name,
            protocol,
        }
    }

    fn file_path(&self) -> &PathBuf {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

impl FileDownloadRequest<Nfs> {
    fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}

fn main() {
    let nfs = Nfs::new(nfs::AuthInfo("nfs::AuthInfo".to_owned()), PathBuf::new());

    let bootp = Bootp();

    let fdr_nfs = FileDownloadRequest::new(PathBuf::new(), nfs);
    let fdr_bootp = FileDownloadRequest::new(PathBuf::new(), bootp);

    fdr_nfs.mount_point();
    // fdr_bootp.mount_point(); // Can't do this
}
