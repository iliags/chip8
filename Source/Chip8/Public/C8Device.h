﻿// See license file

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"
#include "C8Device.generated.h"

DECLARE_DYNAMIC_MULTICAST_DELEGATE(FOnPlaySound);
DECLARE_DYNAMIC_MULTICAST_DELEGATE(FOnStopSound);

class UC8ROM;

UENUM(BlueprintType)
enum class EChip8Key : uint8
{
	Key0 = 0,
	Key1 = 1,
	Key2 = 2,
	Key3 = 3,
	Key4 = 4,
	Key5 = 5,
	Key6 = 6,
	Key7 = 7,
	Key8 = 8,
	Key9 = 9,
	KeyA = 10,
	KeyB = 11,
	KeyC = 12,
	KeyD = 13,
	KeyE = 14,
	KeyF = 15,
	
	MAX UMETA(Hidden)
};

/**
 * Chip 8 Device
 */
UCLASS(Blueprintable, BlueprintType, Category = "Chip8")
class CHIP8_API UC8Device final : public UObject
{
	GENERATED_BODY()

public:
	UC8Device();

	UPROPERTY(BlueprintAssignable, Category = "Chip8")
	FOnPlaySound OnPlaySound;

	UPROPERTY(BlueprintAssignable, Category = "Chip8")
	FOnStopSound OnStopSound;

	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void StartDevice();

	/**
	 * Load a ROM into memory
	 * @param ROM The ROM to load
	 */
	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void LoadROMFromBytes(const TArray<uint8>& ROM);

	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void LoadROMFromBinary(UC8ROM* ROM);

	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void SetKeyState(EChip8Key Key, bool bIsPressed);

	/**
	 * Load the font set into memory
	 */
	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void LoadFont();

	/**
	 * Tick the device
	 * @param DeltaTime The time since the last tick
	 */
	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void Tick(float DeltaTime);

	TArray<uint8> GetMemory() const { return Memory; }
	TArray<uint8> GetRegisters() const { return Registers; }
	TArray<int32> GetVRAM() const { return VRAM; }

	UFUNCTION(BlueprintCallable)
	FString GetVRAMString() const
	{
		FString VRAMString;
		for(int32 i = 0; i < VRAM.Num(); i++)
		{
			if(i % 64 == 0)
			{
				VRAMString += "\n";
			}
			VRAMString += FString::Printf(TEXT("%d"), VRAM[i]);
		}

		return VRAMString;
	}

protected:

	// Device memory
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	TArray<uint8> Memory;

	// Device registers
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	TArray<uint8> Registers;

	// Device VRAM
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	TArray<int32> VRAM;

	// Technically this is 16-bit, but Blueprints don't support uint16
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	TArray<int32> Stack;

	// Keypad state
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	TMap<EChip8Key, uint8> Keys;
	
	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	int32 IndexRegister = 0;

	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	int32 ProgramCounter = 0x200;

	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	uint8 DelayTimer = 0;

	UPROPERTY(VisibleDefaultsOnly, BlueprintReadOnly, Category = "Chip8")
	uint8 SoundTimer = 0;

	UPROPERTY(VisibleDefaultsOnly, BlueprintReadWrite, Category = "Chip8", meta = (ClampMin = "1", ClampMax = "1000", UIMin = "1", UIMax = "1000"))
	int32 CPUSpeed = 50;


	/**
	 * Clears the VRAM
	 */
	UFUNCTION(BlueprintCallable, Category = "Chip8")
	void ClearScreen();

	/**
	 * Set a pixel on the screen
	 * @param X The X coordinate of the pixel
	 * @param Y The Y coordinate of the pixel
	 * @return The new value of the pixel
	 */
	UFUNCTION(BlueprintCallable, Category = "Chip8")
	int32 SetPixel(int32 X, int32 Y);

private:
	UPROPERTY(Transient)
	bool bIsRunning = false;

	/**
	 * Update the timers
	 */
	void UpdateTimers();

	/**
	 * Execute an opcode
	 * @param Opcode The opcode to execute
	 */
	void ExecuteOpcode(uint16 Opcode);
	
};
