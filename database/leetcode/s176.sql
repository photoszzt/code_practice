select MAX(Employee.Salary) as SecondHighestSalary
from Employee
where Salary < (select max(Employee.Salary) from Employee)
