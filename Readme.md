## Procedure 1: Merkle Tree Generation

  **Input**: Original Image `Iorg`
  **Output**: Merkle tree `M`

### Steps
- [x] Take `k` MSBs from each pixel in the original image `Iorg` to create a new image `I`.
- [x] Slice image `I` into non-overlapping blocks `Bi`, where the block size is `N x N` pixels.
- [ ] Encrypt each block `Bi` to get `Bi_e` using encryption `EK(Bi)`.
- [ ] Upload `Bi_e` to IPFS and take the unique hash (fingerprint) as `TXi`, generated by IPFS.
- [ ] Generate Merkle tree `M` by taking `TXi` as the leaf node of the Merkle tree.
- [ ] Store `TXi` and the Merkle Root of `M` in the blockchain.
