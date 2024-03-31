// See license file


#include "C8Device.h"

#include "Data/C8ROM.h"

constexpr int32 SCREEN_WIDTH = 64;
constexpr int32 SCREEN_HEIGHT = 32;

constexpr uint8 FONTSET_OFFSET = 0x0;

constexpr int32 PROGRAM_OFFSET = 0x200;

// Font set
static TArray<uint8> FONT_SET = {
	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
};


UC8Device::UC8Device()
{
	Memory.Init(0, 4096);
	VRAM.Init(0, SCREEN_WIDTH * SCREEN_HEIGHT);
	Registers.Init(0, 16);

	if(!OpcodeMap.IsEmpty())
	{
		OpcodeMap.Empty();
	}

	//OpcodeMap.Add(EChip8Opcode::None, &UC8Device::Nop);
	OpcodeMap.Add(EChip8Opcode::ClearScreen, &UC8Device::ClearScreenNative);
	OpcodeMap.Add(EChip8Opcode::Return, &UC8Device::Return);
	OpcodeMap.Add(EChip8Opcode::Jump, &UC8Device::Jump);
	OpcodeMap.Add(EChip8Opcode::Call, &UC8Device::Call);
	OpcodeMap.Add(EChip8Opcode::SkipIfEqual, &UC8Device::SkipIfEqual);
	OpcodeMap.Add(EChip8Opcode::SkipIfNotEqual, &UC8Device::SkipIfNotEqual);
	OpcodeMap.Add(EChip8Opcode::SkipIfRegistersEqual, &UC8Device::SkipIfRegistersEqual);
	OpcodeMap.Add(EChip8Opcode::SetRegister, &UC8Device::SetRegister);
	OpcodeMap.Add(EChip8Opcode::AddToRegister, &UC8Device::AddToRegister);
	OpcodeMap.Add(EChip8Opcode::SetRegisterToRegister, &UC8Device::SetRegisterToRegister);
	OpcodeMap.Add(EChip8Opcode::OrRegisters, &UC8Device::OrRegisters);
	OpcodeMap.Add(EChip8Opcode::AndRegisters, &UC8Device::AndRegisters);
	OpcodeMap.Add(EChip8Opcode::XORRegisters, &UC8Device::XORRegisters);
	OpcodeMap.Add(EChip8Opcode::AddRegisters, &UC8Device::AddRegisters);
	OpcodeMap.Add(EChip8Opcode::SubtractRegisters, &UC8Device::SubtractRegisters);
	OpcodeMap.Add(EChip8Opcode::ShiftRight, &UC8Device::ShiftRight);
	OpcodeMap.Add(EChip8Opcode::SubtractRegistersReverse, &UC8Device::SubtractRegistersReverse);
	OpcodeMap.Add(EChip8Opcode::ShiftLeft, &UC8Device::ShiftLeft);
	OpcodeMap.Add(EChip8Opcode::SkipIfRegistersNotEqual, &UC8Device::SkipIfRegistersNotEqual);
	OpcodeMap.Add(EChip8Opcode::SetIndexRegister, &UC8Device::SetIndexRegister);
	OpcodeMap.Add(EChip8Opcode::JumpPlusV0, &UC8Device::JumpPlusV0);
	OpcodeMap.Add(EChip8Opcode::Random, &UC8Device::Random);
	OpcodeMap.Add(EChip8Opcode::DrawSprite, &UC8Device::DrawSprite);
	OpcodeMap.Add(EChip8Opcode::SkipIfKeyPressed, &UC8Device::SkipIfKeyPressed);
	OpcodeMap.Add(EChip8Opcode::SkipIfKeyNotPressed, &UC8Device::SkipIfKeyNotPressed);
	OpcodeMap.Add(EChip8Opcode::GetDelayTimer, &UC8Device::GetDelayTimer);
	OpcodeMap.Add(EChip8Opcode::WaitForKeyPress, &UC8Device::WaitForKeyPress);
	OpcodeMap.Add(EChip8Opcode::SetDelayTimer, &UC8Device::SetDelayTimer);
	OpcodeMap.Add(EChip8Opcode::SetSoundTimer, &UC8Device::SetSoundTimer);
	OpcodeMap.Add(EChip8Opcode::AddToIndexRegister, &UC8Device::AddToIndexRegister);
	OpcodeMap.Add(EChip8Opcode::SetIndexRegisterToFont, &UC8Device::SetIndexRegisterToFont);
	OpcodeMap.Add(EChip8Opcode::StoreBCD, &UC8Device::StoreBCD);
	OpcodeMap.Add(EChip8Opcode::StoreRegisters, &UC8Device::StoreRegisters);
	OpcodeMap.Add(EChip8Opcode::LoadRegisters, &UC8Device::LoadRegisters);
	
	LoadFont();
}

void UC8Device::StartDevice()
{
	// TODO: Check everything is loaded
	
	bIsRunning = true;
}

void UC8Device::LoadROMFromBytes(const TArray<uint8>& ROM)
{
	for(int32 i = 0; i < ROM.Num(); i++)
	{
		Memory[i + PROGRAM_OFFSET] = ROM[i];
	}
}

void UC8Device::LoadROMFromBinary(UC8ROM* ROM)
{
	if(!ROM)
	{
		UE_LOG(LogTemp, Warning, TEXT("%s(): ROM is null"), *FString(__FUNCTION__));
		return;
	}

	LoadROMFromBytes(ROM->ROM);
}

void UC8Device::SetKeyState(const EChip8Key Key, const bool bIsPressed)
{
	Keys.Add(Key, bIsPressed);
}

void UC8Device::LoadFont()
{
	for(int32 i = 0; i < FONT_SET.Num(); i++)
	{
		Memory[FONTSET_OFFSET + i] = FONT_SET[i];
	}
}

void UC8Device::Tick(const float DeltaTime)
{
	if(bIsRunning)
	{		
		UpdateTimers();

		for(int32 i = 0; i < CPUSpeed; i++)
		{
			// Fetch opcode
			const uint16 Opcode = (Memory[ProgramCounter] << 8) | Memory[ProgramCounter + 1];

			// Increment the program counter by 2
			ProgramCounter += 2;

			if(ProgramCounter >= Memory.Num())
			{
				UE_LOG(LogTemp, Warning, TEXT("%s(): Program counter out of bounds"), *FString(__FUNCTION__));
				bIsRunning = false;
				break;
			}

			// Decode opcode
			//ExecuteOpcode(Opcode);
			const int32 X = (Opcode & 0x0F00) >> 8;
			const int32 Y = (Opcode & 0x00F0) >> 4;
			const int32 KK = Opcode & 0x00FF;

#if WITH_EDITOR
			//const EChip8Opcode OpcodeType = static_cast<EChip8Opcode>((Opcode & 0xF000) >> 12);
			//const EChip8Opcode OpcodeType = static_cast<EChip8Opcode>((Opcode & 0xF000));

			const EChip8Opcode OpcodeType = [&](uint16 CurrentOpcode) -> EChip8Opcode
			{
				const EChip8Opcode Key = static_cast<EChip8Opcode>((CurrentOpcode & 0xF000));
				
				if(OpcodeMap.Contains(Key))
				{
					return Key;
				}

				if (OpcodeMap.Contains(static_cast<EChip8Opcode>((CurrentOpcode & 0xF000) >> 12)))
				{
					return static_cast<EChip8Opcode>((CurrentOpcode & 0xF000) >> 12);
				}

				return static_cast<EChip8Opcode>(CurrentOpcode);
			}(Opcode);
			
			//UE_LOG(LogTemp, Warning, TEXT("%s(): Opcode 0x%X, %s"), *FString(__FUNCTION__), Opcode, *UEnum::GetValueAsString(OpcodeType));
			

			const FString HasOpcode = OpcodeMap.Contains(OpcodeType) ? "true" : "false";

			UE_LOG(LogTemp, Warning, TEXT("%s(): Opcode 0x%X, %X, %s"), *FString(__FUNCTION__), Opcode, OpcodeType, *HasOpcode);
			
			OpcodeMap[OpcodeType](this, Opcode, X, Y, KK);
			//OpcodeMap.FindRef(OpcodeType)(this, Opcode, X, Y, KK);
#else
			OpcodeMap[static_cast<EChip8Opcode>((Opcode & 0xF000) >> 12)](this, Opcode, X, Y, KK);
#endif
		}
	}	
}

void UC8Device::ClearScreen()
{
	// Note: A memcpy would be faster, but this is more readable
	VRAM.Init(0, SCREEN_WIDTH * SCREEN_HEIGHT);
}

int32 UC8Device::SetPixel(const int32 X, const int32 Y)
{
	const int32 XPos = X % SCREEN_WIDTH;
	const int32 YPos = Y % SCREEN_HEIGHT;

	const int32 PixelIndex = XPos + (YPos * SCREEN_WIDTH);

	VRAM[PixelIndex] ^= 1;

	return VRAM[PixelIndex];
}

void UC8Device::UpdateTimers()
{
	if(DelayTimer > 0)
	{
		DelayTimer--;

		if(DelayTimer == 0)
		{
			// TODO: DelayTimer event
			//UE_LOG(LogTemp, Warning, TEXT("%s(): Delay timer is 0"), *FString(__FUNCTION__));
		}
	}

	if(SoundTimer > 0)
	{
		SoundTimer--;

		// Sounds play while the sound timer is greater than 0
		OnPlaySound.Broadcast();
	}
	else
	{
		OnStopSound.Broadcast();
	}
}

void UC8Device::ClearScreenNative(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->ClearScreen();
}

void UC8Device::Return(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Stack.Num() > 0)
	{
		Device->ProgramCounter = Device->Stack.Pop();
	}
}

void UC8Device::Jump(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->ProgramCounter = Opcode & 0x0FFF;
}

void UC8Device::Call(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Stack.Push(Device->ProgramCounter);
	Device->ProgramCounter = Opcode & 0x0FFF;
}

void UC8Device::SkipIfEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Registers[X] == KK)
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::SkipIfNotEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Registers[X] != KK)
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::SkipIfRegistersEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Registers[X] == Device->Registers[Y])
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::SetRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Registers.IsValidIndex(X))
	{
		Device->Registers[X] = KK;
	}
	else
	{
		UE_LOG(LogTemp, Warning, TEXT("%s(): Invalid register index %d"), *FString(__FUNCTION__), X);
	}
}

void UC8Device::AddToRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] += KK;
}

void UC8Device::SetRegisterToRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] = Device->Registers[Y];
}

void UC8Device::OrRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] |= Device->Registers[Y];
}

void UC8Device::AndRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] &= Device->Registers[Y];
}

void UC8Device::XORRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] ^= Device->Registers[Y];
}

void UC8Device::AddRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	const int32 Result = Device->Registers[X] + Device->Registers[Y];
	Device->Registers[X] = static_cast<uint8>(Result) & 0xFF;
	Device->Registers[0xF] = Result > 255 ? 1 : 0;
}

void UC8Device::SubtractRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	const int32 Result = Device->Registers[X] - Device->Registers[Y];
	Device->Registers[0xF] = Device->Registers[X] >= Device->Registers[Y] ? 1 : 0;
	Device->Registers[X] = static_cast<uint8>(Result) & 0xFF;
}

void UC8Device::ShiftRight(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[0xF] = Device->Registers[X] & 0x1;
	Device->Registers[X] >>= 1;
}

void UC8Device::SubtractRegistersReverse(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	const int32 Result = Device->Registers[Y] - Device->Registers[X];
	Device->Registers[0xF] = Device->Registers[Y] >= Device->Registers[X] ? 1 : 0;
	Device->Registers[X] = static_cast<uint8>(Result) & 0xFF;
}

void UC8Device::ShiftLeft(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[0xF] = (Device->Registers[X] & 0x80) >> 7;
	Device->Registers[X] <<= 1;
}

void UC8Device::SkipIfRegistersNotEqual(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Registers[X] != Device->Registers[Y])
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::SetIndexRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->IndexRegister = Opcode & 0x0FFF;
}

void UC8Device::JumpPlusV0(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->ProgramCounter = (Opcode & 0x0FFF) + Device->Registers[0];
}

void UC8Device::Random(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] = FMath::RandRange(0, 255) & KK;
}

void UC8Device::DrawSprite(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[0xF] = 0;
	const uint8 Height = Opcode & 0x000F;
	
	for(int32 Row = 0; Row < Height; Row++)
	{
		const uint8 Pixel = Device->Memory[Device->IndexRegister + Row];

		for(int32 Col = 0; Col < 8; Col++)
		{
			if((Pixel & (0x80 >> Col)) != 0)
			{
				const int32 PixelX = Device->Registers[X] + Col;
				const int32 PixelY = Device->Registers[Y] + Row;

				const int32 PixelValue = Device->SetPixel(PixelX, PixelY);

				if(PixelValue == 0)
				{
					Device->Registers[0xF] = 1;
				}
			}
		}
	}
}

void UC8Device::SkipIfKeyPressed(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Keys.FindOrAdd(static_cast<EChip8Key>(Device->Registers[X])) != 0)
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::SkipIfKeyNotPressed(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	if(Device->Keys.FindOrAdd(static_cast<EChip8Key>(Device->Registers[X])) == 0)
	{
		Device->ProgramCounter += 2;
	}
}

void UC8Device::GetDelayTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Registers[X] = Device->DelayTimer;
}

void UC8Device::WaitForKeyPress(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	bool bHasKey = false;

	for(auto& Key : Device->Keys)
	{
		if(Key.Value != 0)
		{
			bHasKey = true;
			Device->Registers[X] = static_cast<uint8>(Key.Key);
			break;
		}
	}
				
	if(!bHasKey)
	{
		Device->ProgramCounter -= 2;
	}
}

void UC8Device::SetDelayTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->DelayTimer = Device->Registers[X];
}

void UC8Device::SetSoundTimer(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->SoundTimer = Device->Registers[X];
}

void UC8Device::AddToIndexRegister(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->IndexRegister += Device->Registers[X];
}

void UC8Device::SetIndexRegisterToFont(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->IndexRegister = FONTSET_OFFSET + (Device->Registers[X] * 5);
}

void UC8Device::StoreBCD(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	Device->Memory[Device->IndexRegister] = Device->Registers[X] / 100;
	Device->Memory[Device->IndexRegister + 1] = (Device->Registers[X] / 10) % 10;
	Device->Memory[Device->IndexRegister + 2] = Device->Registers[X] % 10;
}

void UC8Device::StoreRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	for(int32 i = 0; i <= X; i++)
	{
		Device->Memory[Device->IndexRegister + i] = Device->Registers[i];
	}
}

void UC8Device::LoadRegisters(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
	for(int32 i = 0; i <= X; i++)
	{
		Device->Registers[i] = Device->Memory[Device->IndexRegister + i];
	}
}

void UC8Device::Nop(UC8Device* Device, uint16 Opcode, uint8 X, uint8 Y, uint8 KK)
{
}
