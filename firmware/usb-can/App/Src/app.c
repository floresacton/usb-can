#include <app.h>
#include "main.h"
#include "usbd_cdc_if.h"
#include "string.h"

#define MAX_DATA_SIZE 64
#define MAX_PACKET_SIZE MAX_DATA_SIZE+2

extern FDCAN_HandleTypeDef hfdcan2;
extern TIM_HandleTypeDef htim1;

static FDCAN_TxHeaderTypeDef tx_header;
static FDCAN_RxHeaderTypeDef rx_header;
static uint8_t tx_data[64];
static uint8_t rx_data[66];

static const uint8_t dlc_size[16] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64};

void App_Init(void) {
//	FDCAN_FilterTypeDef sFilterConfig;
//
//	sFilterConfig.IdType = FDCAN_STANDARD_ID;
//	sFilterConfig.FilterIndex = 0;
//	sFilterConfig.FilterType = FDCAN_FILTER_MASK;
//	sFilterConfig.FilterConfig = FDCAN_FILTER_TO_RXFIFO0;
//	sFilterConfig.FilterID1 = 0x12;
//	sFilterConfig.FilterID2 = 0x12;
//	if (HAL_FDCAN_ConfigFilter(&hfdcan2, &sFilterConfig) != HAL_OK) {
//		Error_Handler();
//	}
	if (HAL_TIM_Base_Start_IT(&htim1) != HAL_OK) {
		Error_Handler();
	}
	if (HAL_FDCAN_Start(&hfdcan2) != HAL_OK) {
		Error_Handler();
	}

	if (HAL_FDCAN_ActivateNotification(&hfdcan2, FDCAN_IT_RX_FIFO0_NEW_MESSAGE,
			0) != HAL_OK) {
		Error_Handler();
	}

	tx_header.IdType = FDCAN_STANDARD_ID;
	tx_header.TxFrameType = FDCAN_DATA_FRAME;
	tx_header.ErrorStateIndicator = FDCAN_ESI_ACTIVE;
	tx_header.BitRateSwitch = FDCAN_BRS_ON;
	tx_header.FDFormat = FDCAN_FD_CAN;
	tx_header.TxEventFifoControl = FDCAN_NO_TX_EVENTS;
	tx_header.MessageMarker = 0;
}

void App_Loop(void) {
	//Do nothing
}

void App_Send(uint8_t* data) {
	const uint16_t header = *((uint16_t*)data);
	const uint8_t len = header >> 11;
	tx_header.Identifier = header & 0x07FF;
	tx_header.DataLength = len;
	memcpy(tx_data, data+2, len);
	if (HAL_FDCAN_AddMessageToTxFifoQ(&hfdcan2, &tx_header, tx_data) != HAL_OK) {
		Error_Handler();
	}
}

void HAL_FDCAN_RxFifo0Callback(FDCAN_HandleTypeDef *hfdcan, uint32_t RxFifo0ITs) {
	if ((RxFifo0ITs & FDCAN_IT_RX_FIFO0_NEW_MESSAGE) != RESET) {
		if (HAL_FDCAN_GetRxMessage(hfdcan, FDCAN_RX_FIFO0, &rx_header, rx_data+2)
				!= HAL_OK) {
			Error_Handler();
		}

		HAL_GPIO_TogglePin(LED_GPIO_Port, LED_Pin);

		uint16_t* head_pos = (uint16_t*)rx_data;
		*head_pos = (uint16_t)rx_header.Identifier;
		rx_data[1] |= rx_header.DataLength<<3;
		CDC_Transmit_FS(rx_data, 2+dlc_size[rx_header.DataLength]);

		if (HAL_FDCAN_ActivateNotification(hfdcan,
		FDCAN_IT_RX_FIFO0_NEW_MESSAGE, 0) != HAL_OK) {
			Error_Handler();
		}
	}
}

//void HAL_TIM_PeriodElapsedCallback(TIM_HandleTypeDef *htim) {
//	HAL_GPIO_TogglePin(LED_GPIO_Port, LED_Pin);
//}
