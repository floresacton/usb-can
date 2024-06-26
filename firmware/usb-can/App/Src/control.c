#include "control.h"
#include "main.h"

void Control_Init(void) {

}
void Control_Loop(void) {
	HAL_GPIO_TogglePin(LED_GPIO_Port, LED_Pin);
	HAL_Delay(1000);
}
