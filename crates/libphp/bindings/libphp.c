#include "libphp.h"

void libphp_sapi_startup() {
    php_tsrm_startup();
    zend_signal_startup();
}