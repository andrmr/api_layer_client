import asyncio
import py as py_binding


async def main():
    result = await py_binding.get_list()
    print(f"Result: {result}")


asyncio.run(main())
