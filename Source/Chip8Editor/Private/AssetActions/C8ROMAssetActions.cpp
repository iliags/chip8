// See license file


#include "AssetActions/C8ROMAssetActions.h"
#include "Data/C8ROM.h"


UClass* FC8ROMAssetActions::GetSupportedClass() const
{
	return UC8ROM::StaticClass();
}

FText FC8ROMAssetActions::GetName() const
{
	return FText::FromString("Chip8 ROM");
}

FColor FC8ROMAssetActions::GetTypeColor() const
{
	return FColor::Yellow;
}

uint32 FC8ROMAssetActions::GetCategories()
{
	return EAssetTypeCategories::Misc;
}
