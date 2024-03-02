#pragma once

#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"

class FC8ROMAssetActions;

class FChip8EditorModule : public IModuleInterface
{
public:
    virtual void StartupModule() override;
    virtual void ShutdownModule() override;

private:
	TSharedPtr<FC8ROMAssetActions> C8ROMAssetTypeActions;
};
