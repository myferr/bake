class Bake < Formula
  desc "A minimal and fast alternative to Make, written in Rust"
  homepage "https://github.com/myferr/bake"
  version "0.1.2"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/myferr/bake/releases/download/github-releases-v0.1.2/bake-macos-x86_64"
      sha256 "14ed6e706c8ddfb3a20c0f1e9daef1bee5f72ae69381e21ed5e8989d65356aa8"
    elsif Hardware::CPU.arm?
      url "https://github.com/myferr/bake/releases/download/github-releases-v0.1.2/bake-macos-aarch64"
      sha256 "91ee54ba507d69257c2d23ec3e75bb5407b59bf11a9d48d00ec260e87aca4c10"
    end
  end

  on_linux do
    url "https://github.com/myferr/bake/releases/download/github-releases-v0.1.2/bake-linux-x86_64"
    sha256 "e10a1462769e20171c8319cc6804768dcdbbeda91a44258bb83d72c72c2be4b8"
  end

  def install
    bin.install stable.cached_download => "bake"
  end

  test do
    system "#{bin}/bake", "--help"
  end
end
