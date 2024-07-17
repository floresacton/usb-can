#ifndef INC_APP_H_
#define INC_APP_H_

#include "inttypes.h"

void App_Init(void);
void App_Loop(void);

void App_Send(uint8_t* data);

#endif
