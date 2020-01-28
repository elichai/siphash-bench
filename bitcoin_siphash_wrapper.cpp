#include <crypto/siphash.cpp>

extern "C" {

uint64_t SipHashUint256C(uint64_t k0, uint64_t k1, const uint8_t* val) {
    uint256 u256;
    memcpy(u256.begin(), val, u256.size());
  return SipHashUint256(k0, k1, u256);
}

uint64_t SipHashUint256ExtraC(uint64_t k0, uint64_t k1, const uint8_t* val) {
    uint256 u256;
    uint32_t extra;
    memcpy(u256.begin(), val, u256.size());
    memcpy(&extra, val+u256.size(), sizeof(extra));
  return SipHashUint256Extra(k0, k1, u256, extra);
}


uint64_t SipHashNormal(uint64_t k0, uint64_t k1, const unsigned char* data, size_t size) {
    CSipHasher hash(k0, k1);
    hash.Write(data, size);
  return hash.Finalize();
}

}  // extern "C"
