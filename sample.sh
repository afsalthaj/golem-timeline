if git show origin/github_pages:benchmark_cold_storage_large.json > pulled_file.txt; then
  cat pulled_file.txt
else
  echo "Failed to pull the file."
  # Add your alternative actions here
fi
