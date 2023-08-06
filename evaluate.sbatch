#!/usr/bin/env bash
#SBATCH --gpus=1
#SBATCH --output=logs/%j.out

srun nix develop --command ./evaluate.sh
