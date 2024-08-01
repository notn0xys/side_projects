name = input("Enter Employee's name")
no_of_hours = int(input("Enter Employee's name"))
pay_rate = float(input("Enter Employee's name"))
tax_withholding_rate = float(input("Enter Employee's name"))
state_tax_withholding = float(input("Enter Employee's name"))

grosspay = no_of_hours * pay_rate
fed_taxed = grosspay * (1-tax_withholding_rate)
state_taxed = grosspay * (1-state_tax_withholding)
total_taxxed = fed_taxed + state_taxed
net_pay = grosspay - total_taxxed

print("Employee's Name: " + name)
print("Hours worked: ")
print("Pay Rate: " + pay_rate)
print("Gross Pay: {grosspay}")
print("Deductions:")
print("  ")

