class Pytree < Formula
  desc "Clean tree view for Python projects"
  homepage "https://github.com/0x12th/pytree"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/0x12th/pytree/releases/download/v#{version}/pytree-aarch64-apple-darwin.tar.gz"
      # Replace after the GitHub Release uploads this archive.
      sha256 "REPLACE_WITH_SHA256_FOR_AARCH64_APPLE_DARWIN"
    elsif Hardware::CPU.intel?
      url "https://github.com/0x12th/pytree/releases/download/v#{version}/pytree-x86_64-apple-darwin.tar.gz"
      # Replace after the GitHub Release uploads this archive.
      sha256 "REPLACE_WITH_SHA256_FOR_X86_64_APPLE_DARWIN"
    end
  end

  def install
    bin.install "pytree"
  end

  test do
    system "#{bin}/pytree", "--version"
  end
end
