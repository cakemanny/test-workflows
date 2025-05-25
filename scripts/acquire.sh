#!/bin/sh

set -eu

version=4152.2.3
channel=stable
base_name=flatcar_production_qemu_uefi

case "$(uname -m)" in
    aarch64|arm64)  arch=arm64;;
    x86_64)         arch=amd64;;
    *)
        echo "unknown arch: $(uname -m)" 1>&2
        exit 1
        ;;
esac

for suff in .sh _image.img _efi_code.fd _efi_vars.fd; do
  wget https://${channel}.release.flatcar-linux.net/${arch}-usr/$version/${base_name}${suff}
  wget https://${channel}.release.flatcar-linux.net/${arch}-usr/$version/${base_name}${suff}.sig
  gpg --verify ${base_name}${suff}.sig
done

chmod +x ${base_name}.sh

