#include "Chip8Editor.h"

#include "AssetActions/C8ROMAssetActions.h"

#define LOCTEXT_NAMESPACE "FChip8EditorModule"

void FChip8EditorModule::StartupModule()
{
	C8ROMAssetTypeActions = MakeShared<FC8ROMAssetActions>();
	FAssetToolsModule::GetModule().Get().RegisterAssetTypeActions(C8ROMAssetTypeActions.ToSharedRef());
}

void FChip8EditorModule::ShutdownModule()
{
	if (!FModuleManager::Get().IsModuleLoaded("AssetTools"))
	{
		return;
	}
	FAssetToolsModule::GetModule().Get().UnregisterAssetTypeActions(C8ROMAssetTypeActions.ToSharedRef());
}

#undef LOCTEXT_NAMESPACE
    
IMPLEMENT_MODULE(FChip8EditorModule, Chip8Editor)