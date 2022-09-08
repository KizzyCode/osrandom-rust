// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>
#include <Security/SecRandom.h>


/**
 * @brief The name of the current provider
 * 
 * @return The name of the current provider
 */
const char* osrandom_provider() {
    return __FILE__;
}


/**
 * @brief Get a series of cryptographically secure random bytes
 * 
 * @param buf The buffer to write the bytes to
 * @param len The amount of bytes to write
 * @return 0 on success or 1 in case of an error 
 */
uint8_t osrandom_native(uint8_t* buf, size_t len) {
	// Get the random bytes
	if (SecRandomCopyBytes(kSecRandomDefault, len, buf) != errSecSuccess) {
		return 1;
	}

	// Return success
	return 0;
}
