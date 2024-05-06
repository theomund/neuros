# NeurOS - Hobbyist operating system written in Rust.
# Copyright (C) 2024 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

FROM registry.fedoraproject.org/fedora:40
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN dnf install -y \
    dnf-plugins-core-4.6.0 \
    edk2-ovmf-20240214 \
    gcc-14.0.1 \
    gdb-14.2 \
    git-2.45.0 \
    hadolint-2.12.0 \
    make-4.4.1 \
    qemu-8.2.2 \
    xorriso-1.5.6 \
    && dnf clean all
RUN dnf copr enable -y mczernek/vale \
    && dnf install -y vale-3.4.1 \
    && dnf clean all
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2024-05-05 -t x86_64-unknown-none
ENV PATH="$PATH:/root/.cargo/bin"
WORKDIR /usr/src/app
