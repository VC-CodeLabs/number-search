namespace NumberTumbler
{
    internal class Program
    {
        static void Main(string[] args)
        {
            string filePath = "./sampleFile.txt";
            if (args.Length > 0)
                filePath = args[0];

            try
            {
                FileStream file = new(filePath, FileMode.Open);

                int? num1 = null;
                int? num2 = null;

                byte[] buffer = new byte[4096];
                while (file.Read(buffer, 0, buffer.Length) > 0)
                {
                    foreach (string c in buffer.Select(c => char.ConvertFromUtf32(c)))
                    {
                        if (int.TryParse(c, out int i))
                        {
                            if (num1 == null)
                                num1 = num2 = i;
                            else
                                num2 = i;
                        }
                        if (c == "\n")
                        {
                            if (num1 != null)
                                Console.WriteLine($"{num1}{num2}");
                            num1 = num2 = null;
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
