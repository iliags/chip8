// See license file


#include "Factory/C8ROMFactory.h"
#include "Data/C8ROM.h"

UC8ROMFactory::UC8ROMFactory(const FObjectInitializer& ObjectInitializer) : Super(ObjectInitializer)
{
	Formats.Add(TEXT("ch8;Chip8 ROM"));
	
	SupportedClass = UC8ROM::StaticClass();

	bCreateNew = false;
	bEditAfterNew = false;
	bEditorImport = true;
}

UObject* UC8ROMFactory::FactoryCreateBinary(UClass* InClass, UObject* InParent, FName InName, EObjectFlags Flags,
	UObject* Context, const TCHAR* Type, const uint8*& Buffer, const uint8* BufferEnd, FFeedbackContext* Warn,
	bool& bOutOperationCanceled)
{
	const TObjectPtr<UC8ROM> NewROM = NewObject<UC8ROM>(InParent, InClass, InName, Flags);
	TArray<uint8> Result;
	
	if(FFileHelper::LoadFileToArray(NewROM->ROM, *CurrentFilename))
	{
		//UE_LOG(LogTemp, Warning, TEXT("%s(): Loaded file"), *FString(__FUNCTION__));
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("%s(): Failed to load file"), *FString(__FUNCTION__));
	}

	bOutOperationCanceled = false;
	
	return NewROM;
}

bool UC8ROMFactory::ShouldShowInNewMenu() const
{
	return true;
}

bool UC8ROMFactory::FactoryCanImport(const FString& Filename)
{
	const FString Extension = FPaths::GetExtension(Filename);

	return Extension == TEXT("ch8");
}
