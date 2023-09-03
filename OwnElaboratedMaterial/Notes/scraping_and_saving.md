- If you are triggering your scraper with a scheduler on **non-realtime** systems, 
sometimes, it can be tricky to call your code in the required tolerance.  
- Also, you must handle internet blackouts by running a redundant system of at least 
3 participant at different geological locations (ideally).  
- Finally, keep your database size quite constant to be able to scale your scraper and optimize resource usage. 
Like, you can split your data into 1 day sequences if it is updated roughly in every second and contains only about 10-200 bytes per frame.