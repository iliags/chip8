// See license file

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "C8Visualizer.generated.h"

class UC8ROM;
class UC8Device;
class UInstancedStaticMeshComponent;

UCLASS()
class CHIP8_API AC8Visualizer : public AActor
{
	GENERATED_BODY()

public:
	AC8Visualizer();
	virtual void Tick(float DeltaTime) override;

protected:
	virtual void BeginPlay() override;
	virtual void OnConstruction(const FTransform& Transform) override;

	UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Visualizer")
	TObjectPtr<USceneComponent> SceneComponent;
	
	UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Visualizer")
	TObjectPtr<UInstancedStaticMeshComponent> VisualizerMesh;

	UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Visualizer")
	TObjectPtr<UC8ROM> TestROM;

	UPROPERTY(BlueprintReadOnly, Category = "Visualizer")
	TObjectPtr<UC8Device> Device;

private:
	
	
};
