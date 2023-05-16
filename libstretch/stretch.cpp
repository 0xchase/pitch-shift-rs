#include "stretch.h"
#include <stdlib.h>
#include <stdio.h>

extern "C" void temp() {
    auto stretch = signalsmith::stretch::SignalsmithStretch<float>();
}

int main() {
    temp();
    return 0;
}