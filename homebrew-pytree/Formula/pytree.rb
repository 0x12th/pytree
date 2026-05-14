class Pytree < Formula
  desc "Clean tree view for Python projects"
  homepage "https://github.com/0x12th/pytree"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/0x12th/pytree/releases/download/v0.1.0/pytree-v0.1.0-macos-arm64.tar.gz"
      sha256 "d51d84fa8da12c0c82f72c8c4e24da08ed4789e821922a7775c6e467c3975374"
    elsif Hardware::CPU.intel?
      url "https://github.com/0x12th/pytree/releases/download/v0.1.0/pytree-v0.1.0-macos-x86_64.tar.gz"
      sha256 "90bd1a6446ed905e5f520abb54de0c5dbbe47cbb1b8d4d1cc02f6dbeaa81fbd9"
    end
  end

  def install
    bin.install "pytree"
  end

  test do
    system bin/"pytree", "--version"
  end
end
