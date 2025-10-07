#ifndef INC_APP_H_
#define INC_APP_H_

#include "inttypes.h"

void App_Init(void);
void App_Update(void);

void App_Send(uint8_t* data, uint8_t len);

#endif
