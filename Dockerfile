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

FROM registry.fedoraproject.org/fedora:39
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN dnf install -y \
    dnf-plugins-core-4.5.0 \
    edk2-ovmf-20231122 \
    gcc-13.2.1 \
    git-2.44.0 \
    hadolint-2.12.0 \
    make-4.4.1 \
    xorriso-1.5.6 \
    && dnf clean all
RUN dnf copr enable -y mczernek/vale \
    && dnf install -y vale-3.0.7 \
    && dnf clean all
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /usr/src/app
