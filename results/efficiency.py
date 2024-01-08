import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Read data
df = pd.read_csv('referee_schedule.csv')

# Example: Calculate gaps (this will depend on your data's format)
df['gap'] = (df['referee_time'] - df['match_time']).abs()

# Plotting
plt.figure(figsize=(10, 6))
sns.histplot(df['gap'], bins=30)
plt.title('Distribution of Time Gaps Between Matches and Refereeing')
plt.xlabel('Gap Time')
plt.ylabel('Frequency')
plt.show()

