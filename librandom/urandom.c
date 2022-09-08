#include <unistd.h>
#include <fcntl.h>
#include <stdint.h>


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
    // Open /dev/urandom
    int urandom = open("/dev/urandom", O_CLOEXEC, O_RDONLY);
    if (urandom < 0) {
        return 1;
    }
    
    // Read random bytes
    while (len) {
        // Read random bytes
        ssize_t bytes_read = read(urandom, buf, len);
        if (bytes_read < 1) {
            close(urandom);
            return 1;
        }
        
        // Adjust buffer and remaining length
        buf += bytes_read;
        len -= bytes_read;
    }
    
    // Close /dev/urandom and return success
    close(urandom);
    return 0;
}
