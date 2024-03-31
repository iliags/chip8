// See license file

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

UENUM()
enum class EChip8Opcode : uint16
{
	//None = 0x0000,
	ClearScreen = 0x00E0,
	Return = 0x00EE,
	Jump = 0x1000,
	Call = 0x2000,
	SkipIfEqual = 0x3000,
	SkipIfNotEqual = 0x4000,
	SkipIfRegistersEqual = 0x5000,
	SetRegister = 0x6000,
	AddToRegister = 0x7000,
	SetRegisterToRegister = 0x8000,
	OrRegisters = 0x8001,
	AndRegisters = 0x8002,
	XORRegisters = 0x8003,
	AddRegisters = 0x8004,
	SubtractRegisters = 0x8005,
	ShiftRight = 0x8006,
	SubtractRegistersReverse = 0x8007,
	ShiftLeft = 0x800E,
	SkipIfRegistersNotEqual = 0x9000,
	SetIndexRegister = 0xA000,
	JumpPlusV0 = 0xB000,
	Random = 0xC000,
	DrawSprite = 0xD000,
	SkipIfKeyPressed = 0xE09E,
	SkipIfKeyNotPressed = 0xE0A1,
	GetDelayTimer = 0xF007,
	WaitForKeyPress = 0xF00A,
	SetDelayTimer = 0xF015,
	SetSoundTimer = 0xF018,
	AddToIndexRegister = 0xF01E,
	SetIndexRegisterToFont = 0xF029,
	StoreBCD = 0xF033,
	StoreRegisters = 0xF055,
	LoadRegisters = 0xF065,
	
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

	TMap<EChip8Opcode, void(*)(UC8Device*, uint16, uint8, uint8, uint8)> OpcodeMap;

	static void ClearScreenNative(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void Return(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void Jump(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void Call(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfNotEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfRegistersEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void AddToRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetRegisterToRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void OrRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void AndRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void XORRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void AddRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SubtractRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void ShiftRight(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SubtractRegistersReverse(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void ShiftLeft(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfRegistersNotEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetIndexRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void JumpPlusV0(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void Random(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void DrawSprite(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfKeyPressed(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SkipIfKeyNotPressed(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void GetDelayTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void WaitForKeyPress(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetDelayTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetSoundTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void AddToIndexRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void SetIndexRegisterToFont(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void StoreBCD(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void StoreRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void LoadRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
	static void Nop(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK);
};
