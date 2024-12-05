import json
import pandas as pd

def laad_training_data(json_file, model_name):
    with open(json_file, 'r') as file:
        data = json.load(file)
    steps = data['x']
    losses = data['y']
    df = pd.DataFrame({
        'Step': steps,
        model_name: losses
    })
    return df

file_model1 = "llm_1_training.json" 
file_model2 = "llm_2_training.json"

df_model1 = laad_training_data(file_model1, "Loss_LLM1")
df_model2 = laad_training_data(file_model2, "Loss_LLM2")

combined_df = pd.merge(df_model1, df_model2, on="Step", how="outer")

output_file = "verwerkte_training_data.csv"
combined_df.to_csv(output_file, index=False)

print(f"Data samengevoegd in: {output_file}")
