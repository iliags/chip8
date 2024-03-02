// See license file


#include "C8Visualizer.h"

#include "C8Device.h"
#include "Components/InstancedStaticMeshComponent.h"

AC8Visualizer::AC8Visualizer()
{
	PrimaryActorTick.bCanEverTick = true;

	SceneComponent = CreateDefaultSubobject<USceneComponent>(TEXT("SceneComponent"));
	RootComponent = SceneComponent;
	
	VisualizerMesh = CreateDefaultSubobject<UInstancedStaticMeshComponent>(TEXT("VisualizerMesh"));
	VisualizerMesh->SetupAttachment(RootComponent);
	VisualizerMesh->SetNumCustomDataFloats(1);
}

void AC8Visualizer::BeginPlay()
{
	Super::BeginPlay();

	Device = NewObject<UC8Device>();

	if(TestROM)
	{
		Device->LoadROMFromBinary(TestROM);
		Device->StartDevice();
	}
}

void AC8Visualizer::OnConstruction(const FTransform& Transform)
{
	Super::OnConstruction(Transform);

	VisualizerMesh->ClearInstances();

	TArray<FTransform> Transforms;
	for(int32 i = 0; i < 64*32; i++)
	{
		const int32 Y = (i % 64);
		const int32 X = 31-(i / 64);
		const FTransform NewTransform = FTransform(FVector(X * 100, Y * 100, 0));
		Transforms.Add(NewTransform);
	}
	
	VisualizerMesh->AddInstances(Transforms, false);

	const FVector NewLocation = -FVector((31*100)/2,(63*100)/2, 0);
	VisualizerMesh->SetRelativeLocation(NewLocation);
}

void AC8Visualizer::Tick(const float DeltaTime)
{
	Super::Tick(DeltaTime);

	if(Device)
	{
		Device->Tick(DeltaTime);

		const TArray<int32> VRAM = Device->GetVRAM();
		for(int32 i = 0; i < VRAM.Num(); i++)
		{
			const bool bIsLast = i == VRAM.Num() - 1;
			VisualizerMesh->SetCustomDataValue(i, 0, VRAM[i], bIsLast);
		}
	}
}

