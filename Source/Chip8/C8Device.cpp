// See license file


#include "C8Device.h"

constexpr int32 SCREEN_WIDTH = 64;
constexpr int32 SCREEN_HEIGHT = 32;

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
}

void UC8Device::LoadROM(const TArray<uint8>& ROM)
{
	for(int32 i = 0; i < ROM.Num(); i++)
	{
		Memory[i + 0x200] = ROM[i];
	}
}

void UC8Device::LoadFont(const int32 Offset)
{
	for(int32 i = 0; i < FONT_SET.Num(); i++)
	{
		Memory[i + Offset] = FONT_SET[i];
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

			// Decode opcode
			ExecuteOpcode(Opcode);
		}
	}
	else if(WaitingForKey != EChip8Key::MAX)
	{
		if(Keys.FindOrAdd(WaitingForKey) != 0)
		{
			Registers[static_cast<int32>(WaitingForKey)] = Keys[WaitingForKey];
			WaitingForKey = EChip8Key::MAX;
			bIsRunning = true;
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
	const int32 PixelIndex = WrapPixel(X, SCREEN_WIDTH) +
		(WrapPixel(Y, SCREEN_HEIGHT) * SCREEN_WIDTH);

	VRAM[PixelIndex] ^= 1;

	return VRAM[PixelIndex];
}

int32 UC8Device::WrapPixel(const int32 Location, const int32 MaxValue)
{
	if(Location < 0)
	{
		return Location + MaxValue;
	}

	if(Location >= MaxValue)
	{
		return Location - MaxValue;
	}

	return Location;
}

void UC8Device::UpdateTimers()
{
	if(DelayTimer > 0)
	{
		DelayTimer--;

		if(DelayTimer == 0)
		{
			// TODO: DelayTimer event
			UE_LOG(LogTemp, Warning, TEXT("%s(): Delay timer is 0"), *FString(__FUNCTION__));
		}
	}

	if(SoundTimer > 0)
	{
		SoundTimer--;

		// Sounds play while the sound timer is greater than 0
		PlaySound();
	}
}

void UC8Device::ExecuteOpcode(const uint16 Opcode)
{
	// Increment the program counter by 2
	ProgramCounter += 2;

	// Upper bits
	const uint8 X = (Opcode & 0x0F00) >> 8;

	// Lower bits
	const uint8 Y = (Opcode & 0x00F0) >> 4;

	// Common lower bits
	const uint8 NN = Opcode & 0x00FF;
	
	switch(Opcode)
	{
		case 0x0000:
			switch (Opcode) {
				case 0x00E0:
						ClearScreen();
					break;
				case 0x00EE:
						// Return from a subroutine
						ProgramCounter = Stack.Pop();
					break;
				default: 
						UE_LOG(LogTemp, Warning, TEXT("%s(): Unknown 0x0000 Opcode 0x%X"), *FString(__FUNCTION__), Opcode);
					break;
			}

			break;
		case 0x1000:
				// Jump to address NNN
				ProgramCounter = Opcode & 0xFFF;
			break;
		case 0x2000:
				// Call subroutine at NNN
				Stack.Push(ProgramCounter);
				ProgramCounter = Opcode & 0xFFF;
			break;
		case 0x3000:
				// Skip next instruction if Vx == NN
				if(Registers[X] == NN)
				{
					ProgramCounter += 2;
				}
			break;
		case 0x4000:
				// Skip next instruction if Vx != NN
				if(Registers[X] != NN)
				{
					ProgramCounter += 2;
				}
			break;
		case 0x5000:
				// Skip next instruction if Vx == Vy
				if(Registers[X] == Registers[Y])
				{
					ProgramCounter += 2;
				}
			break;
		case 0x6000:
				// Set Vx to NN
				Registers[X] = NN;	
			break;
		case 0x7000:
				// Add NN to Vx
				Registers[X] = FMath::Wrap(Registers[X]+NN, 0, 255);
				
			break;
		case 0x8000:
			switch (Opcode & 0xF) {
				case 0x0:
						// Set Vx to Vy
						Registers[X] = Registers[Y];	
					break;
				case 0x1:
						// Set Vx to Vx | Vy
						Registers[X] |= Registers[Y];	
					break;
				case 0x2:
						// Set Vx to Vx & Vy
						Registers[X] &= Registers[Y];	
					break;
				case 0x3:
						// Set Vx to Vx ^ Vy
						Registers[X] ^= Registers[Y];	
					break;
				case 0x4:
						// Add Vy to Vx, set VF to 1 if there's a carry
						Registers[0xF] = (Registers[X] + Registers[Y] > 255) ? 1 : 0;
						Registers[X] = FMath::Wrap(Registers[X]+ Registers[Y], 0, 255);
					break;
				case 0x5:
						// Subtract Vy from Vx, set VF to 0 if there's a borrow
						Registers[0xF] = (Registers[X] > Registers[Y]) ? 1 : 0;
						Registers[X] = FMath::Wrap(Registers[X] - Registers[Y], 0, 255);
					break;
				case 0x6:
						// Shift Vx right by 1, set VF to the least significant bit of Vx before the shift
						Registers[0xF] = Registers[X] & 0x1;
						Registers[X] >>= 1;
					break;
				case 0x7:
						// Set Vx to Vy - Vx, set VF to 0 if there's a borrow
						Registers[0xF] = (Registers[Y] > Registers[X]) ? 1 : 0;
						Registers[X] = FMath::Wrap(Registers[Y] - Registers[X], 0, 255);
					break;
				case 0xE:
						// Shift Vx left by 1, set VF to the most significant bit of Vx before the shift
						Registers[0xF] = (Registers[X] & 0x80) >> 7;
						Registers[X] <<= 1;
					break;
				default: 
					UE_LOG(LogTemp, Warning, TEXT("%s(): Unknown 0x8000 Opcode 0x%X"), *FString(__FUNCTION__), Opcode);
					break;
			}
		
			break;
		case 0x9000:
				// Skip next instruction if Vx != Vy
				if(Registers[X] != Registers[Y])
				{
					ProgramCounter += 2;
				}
			break;
		case 0xA000:
				// Set index register to NNN
				Registers[X] = Opcode & 0xFFF;
			break;
		case 0xB000:
				// Jump to address NNN + V0
				ProgramCounter = (Opcode & 0xFFF) + Registers[0];	
			break;
		case 0xC000:
				// Set Vx to a random number & NN
				Registers[X] = FMath::RandRange(0, 255) & NN;
			break;
		case 0xD000:
				// Draw a sprite at position Vx, Vy with N bytes of sprite data starting at the address stored in I
				Registers[0xF] = 0;

				for(int32 Row = 0; Row < (Opcode & 0xF); Row++)
				{
					const uint8 Pixel = Memory[IndexRegister + Row];

					for(int32 Col = 0; Col < 8; Col++)
					{
						if((Pixel & (0x80 >> Col)) != 0)
						{
							const int32 PixelX = Registers[X] + Col;
							const int32 PixelY = Registers[Y] + Row;

							const int32 PixelValue = SetPixel(PixelX, PixelY);

							if(PixelValue == 0)
							{
								Registers[0xF] = 1;
							}
						}
					}
				}
			break;
		case 0xE000:
			switch (Opcode & 0xFF) {
				case 0x9E:
						// Skip next instruction if key with the value of Vx is pressed
						if(Keys.FindOrAdd(static_cast<EChip8Key>(Registers[X])) != 0)
						{
							ProgramCounter += 2;
						}
					break;
				case 0xA1:
						// Skip next instruction if key with the value of Vx is not pressed
						if(Keys.FindOrAdd(static_cast<EChip8Key>(Registers[X])) == 0)
						{
							ProgramCounter += 2;
						}	
					break;
				default: 
					UE_LOG(LogTemp, Warning, TEXT("%s(): Unknown 0xE000 Opcode 0x%X"), *FString(__FUNCTION__), Opcode);
					break;
			}

			break;
		case 0xF000:
			switch (Opcode & 0xFF) {
				case 0x07:
						// Set Vx to the value of the delay timer
						Registers[X] = DelayTimer;	
					break;
				case 0x0A:
						// Wait for a key press, store the value of the key in Vx
						bIsRunning = false;
						// TODO: Wait for key press event
						WaitingForKey = static_cast<EChip8Key>(X);
						UE_LOG(LogTemp, Warning, TEXT("%s(): Waiting for key"), *FString(__FUNCTION__));
						//bIsRunning = true;
					break;
				case 0x15:
						// Set the delay timer to Vx
						DelayTimer = Registers[X];	
					break;
				case 0x18:
						// Set the sound timer to Vx
						SoundTimer = Registers[X];
					break;
				case 0x1E:
						// Add Vx to the index register
						IndexRegister += Registers[X];	
					break;
				case 0x29:
						// Set I to the location of the sprite for the character in Vx
						IndexRegister = Registers[X] * 5;	
					break;
				case 0x33:
						// Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
						Memory[IndexRegister] = Registers[X] / 100;
						Memory[IndexRegister + 1] = (Registers[X] / 10) % 10;
						Memory[IndexRegister + 2] = Registers[X] % 10;	
					break;
				case 0x55:
						// Store V0 to Vx in memory starting at address I
						for(int32 i = 0; i <= X; i++)
						{
							Memory[IndexRegister + i] = Registers[i];
						}
					break;
				case 0x65:
						// Read V0 to Vx from memory starting at address I
						for(int32 i = 0; i <= X; i++)
						{
							Registers[i] = Memory[IndexRegister + i];
						}
					break;
				default: 
					UE_LOG(LogTemp, Warning, TEXT("%s(): Unknown 0xF000 Opcode 0x%X"), *FString(__FUNCTION__), Opcode);
					break;
			}

			break;
		default:
			UE_LOG(LogTemp, Warning, TEXT("%s(): Unknown Opcode 0x%X"), *FString(__FUNCTION__), Opcode);
			break;
	}
}
