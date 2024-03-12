// See license file

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"
#include "C8ROM.generated.h"

/**
 * 
 */
UCLASS(BlueprintType, hidecategories=(Object))
class CHIP8_API UC8ROM final: public UObject
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Chip8")
	TArray<uint8> ROM;
};
