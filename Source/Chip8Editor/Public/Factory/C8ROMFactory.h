// See license file

#pragma once

#include "CoreMinimal.h"
#include "Factories/Factory.h"
#include "C8ROMFactory.generated.h"

/**
 * 
 */
UCLASS()
class CHIP8EDITOR_API UC8ROMFactory final : public UFactory
{
	GENERATED_BODY()

public:
	UC8ROMFactory(const FObjectInitializer& ObjectInitializer);

	// UFactory interface
	virtual UObject* FactoryCreateBinary(UClass* InClass, UObject* InParent, FName InName, EObjectFlags Flags, UObject* Context, const TCHAR* Type, const uint8*& Buffer, const uint8* BufferEnd, FFeedbackContext* Warn, bool& bOutOperationCanceled) override;
	virtual bool ShouldShowInNewMenu() const override;
	virtual bool FactoryCanImport(const FString& Filename) override;
	// End of UFactory interface
};
